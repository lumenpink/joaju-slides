# 22_presentation_slides_and_handouts.md

## 1. Visão Geral e Objetivos

Este módulo define o subsistema de apresentação e geração de materiais didáticos a partir de arquivos Markdown unificados no Mandu[span_0](start_span)[span_0](end_span). 

O objetivo é permitir que uma única nota de aula ou documento técnico sirva como fonte para três saídas distintas compiladas via Typst (`17_tui_typst_rendering_and_buffer_safety.md`)[span_1](start_span)[span_1](end_span):
1. **Modo Projeção (`--slides`):** Lâminas visuais de proporção fixa (16:9 ou 4:3), tipografia expandida e baixa densidade informacional (Método Lessig/Takahashi).
2. **Modo Apostila (`--handout`):** Folhas A4 contendo a lâmina projetada no terço superior e notas aprofundadas renderizadas via Joaju Engine (`08_joaju_preview_engine.md`)[span_2](start_span)[span_2](end_span) na seção inferior.
3. **Modo Roteiro/Notas (`--notes`):** Documento corrido textual contendo apenas os comentários aprofundados, convertendo títulos de lâminas em cabeçalhos de seção tradicionais.

---

## 2. Sintaxe e Delimitação no Documento

A separação entre lâminas e áreas de comentário segue uma sintaxe limpa e sem dependências externas, compatível com o parser base (`02_vault_scanner_and_parser.md`)[span_3](start_span)[span_3](end_span).

### 2.1. Estrutura Canônica

* **Delimitador de Lâmina (`---`):** Marca a transição entre lâminas consecutivas. O primeiro bloco `---` no início do arquivo é reservado exclusivamente para o Frontmatter[span_4](start_span)[span_4](end_span).
* **Delimitador de Notas/Comentários (`???`):** Linha isolada contendo exatamente três pontos de interrogação. Todo o conteúdo abaixo desta linha até o próximo `---` pertence ao contexto dos comentários Joaju.
* **Frontmatter:** Suporta chaves específicas de formatação e bibliografia[span_5](start_span)[span_5](end_span).

```markdown
---
title: Fisiologia Cardiovascular - Ciclo Cardíaco
format: "16:9"
bibliography: "referencias.bib"
bib_style: "abnt"
---

![[diagrama_wiggers.svg]]
Fechamento Mitral e Fase Isovolumétrica

???
### Mecanismo Fisiológico
O fechamento da **valva mitral** marca o início da sístole ventricular. Neste ponto:
1. Ocorre a vibração das cúspides e estruturas adjacentes, gerando o componente principal de **B1**.
2. A pressão intraventricular eleva-se bruscamente sem alteração de volume (contração isovolumétrica):

$$ \frac{dP}{dt}_{max} $$

> [!warning] Relevância Clínica
> Se a complacência ventricular estiver reduzida, a contração atrial prévia gera a quarta bulha (**B4**). Conforme demonstrado por [@guyton2021], a monitorização hemodinâmica antecipa descompensações. Veja mais detalhes em [[Semiologia Cardiovascular - Bulhas]].

---

# B3 (Terceira Bulha)
Sobrecarga de volume ventricular

???
A terceira bulha ocorre no início da diástole durante o enchimento ventricular rápido. Em pacientes jovens ou atletas pode ser fisiológica; em idosos traduz insuficiência ventricular sistólica.
```

---

## 3. Modelo de Dados e Representação Intermediária (IR)

O scanner estende o parser principal (`02_vault_scanner_and_parser.md`)[span_6](start_span)[span_6](end_span) adicionando a decomposição em blocos de apresentação:

```rust
use std::path::PathBuf;

pub struct SlideDeck {
    pub title: String,
    pub aspect_ratio: SlideAspectRatio,
    pub bibliography: Option<PathBuf>,
    pub bib_style: String,
    pub slides: Vec<SlideBlock>,
}

pub enum SlideAspectRatio {
    Widescreen16_9,
    Standard4_3,
    A4Landscape,
}

pub struct SlideBlock {
    pub slide_index: usize,
    pub title_hint: Option<String>,
    pub visual_content_ast: Vec<JoajuNode>, // Processado pelo parser visual
    pub speaker_notes_ast: Option<Vec<JoajuNode>>, // Processado pelo motor Joaju completo
}
```

---

## 4. Pipeline de Compilação Typst (`17_tui_typst_rendering_and_buffer_safety.md`)

O Mandu converte os blocos AST diretamente para arquivos de marcação Typst temporários com tratamento rigoroso de escape de texto e segurança de buffer (`buffer_safety`)[span_7](start_span)[span_7](end_span).

### 4.1. Modo Projeção (`mandu build --slides <arquivo.md>`)

* **Configuração de Página:**
  ```typst
  #set page(
    paper: "presentation-16-9",
    margin: (x: 2cm, y: 1.5cm)
  )
  #set text(font: "Liberation Sans", size: 28pt)
  #set align(center + horizon)
  ```
* Cada `SlideBlock.visual_content_ast` é renderizado de forma centralizada.
* Imagens recebem contenção automática de escala (`height: 70%`, `fit: "contain"`).
* Insere `#pagebreak()` entre cada lâmina.
* As notas (`speaker_notes_ast`) são completamente ignoradas neste modo.

### 4.2. Modo Apostila / Handout (`mandu build --handout <arquivo.md>`)

* **Configuração de Página:** Folha A4 vertical (`paper: "a4"`), margens de 2 cm.
* **Geometria de Bloco:**
  * No topo de cada página inicial de lâmina, renderiza um quadro com contorno suave (`stroke: 0.5pt + luma(180)`) e fundo neutro contendo a visualização da lâmina em miniatura proporcional (altura máxima de 7 cm).
  * Abaixo da miniatura, insere um separador e o bloco completo de `speaker_notes_ast` processado pelo motor Joaju (suportando tabelas, blocos de código, fórmulas matemáticas e referências cruzadas)[span_8](start_span)[span_8](end_span).
* **Gestão de Transbordo (Overflow):**
  * Se o comentário Joaju ultrapassar o limite inferior da folha A4 atual, o Typst quebra a página naturalmente.
  * Na página de continuação, é emitido um micro-cabeçalho no topo:
    ```typst
    #align(right)[#text(size: 8pt, fill: luma(120), italic: true)[(Continuação das notas - Lâmina #slide_index: #title)]]
    ```
  * A próxima lâmina do documento **obriga sempre** a inserção de um `#pagebreak()` explícito para iniciar isolada no topo de uma nova folha.

### 4.3. Modo Roteiro / Apenas Comentários (`mandu build --notes <arquivo.md>`)

* **Configuração de Página:** Folha A4 corrida tradicional com paginação inferior centralizada.
* **Conversão Estrutural:**
  * O conteúdo puramente visual do slide é omitido.
  * O título da lâmina (`title_hint` ou cabeçalho nível 1 extraído) é promovido a um título de seção padrão (`= Seção`).
  * O conteúdo de `speaker_notes_ast` flui de forma contínua, sem forçar quebras de página por lâmina.

---

## 5. Gestão de Bibliografia e Citações

1. **Resolução de Fontes:** As citações no formato `[@chave]` ou `[[referencias#chave]]` são mapeadas contra o arquivo `.bib` declarado no frontmatter (`bibliography: "referencias.bib"`)[span_9](start_span)[span_9](end_span).
2. **Posicionamento:**
   * **Em `--slides`:** As fontes citadas durante a apresentação são compiladas em uma lâmina final dedicada (ou múltiplas, se necessário), com tipografia reduzida (`size: 14pt`).
   * **Em `--handout` e `--notes`:** A bibliografia é compilada como a última seção formal do documento após a última lâmina, gerada nativamente pelo Typst via `#bibliography("referencias.bib", style: "abnt")`.

---

## 6. Integração com CLI Dispatcher (`03_dispatcher_cli.md`) e Linter (`06_linter_and_generators.md`)

### 6.1. Comandos do CLI Dispatcher[span_10](start_span)[span_10](end_span)

```bash
# Gera apenas a apresentação de slides (16:9 PDF)
mandu build --slides aula_01.md -o aula_01_apresentacao.pdf

# Gera a apostila comentada completa para os alunos (A4 PDF)
mandu build --handout aula_01.md -o aula_01_apostila.pdf

# Gera apenas o texto contínuo dos comentários Joaju (A4 PDF)
mandu build --notes aula_01.md -o aula_01_roteiro.pdf
```

### 6.2. Regras de Validação do Linter (`06_linter_and_generators.md`)[span_11](start_span)[span_11](end_span)

* `WARN_SLIDE_OVERFLOW`: Emite aviso caso o conteúdo visual da lâmina exceda a altura estimada da moldura fixa de proporção (indicando que há texto em excesso no slide).
* `WARN_HANDOUT_OVERFLOW`: Emite aviso informativo indicando que os comentários da lâmina $N$ ultrapassaram uma única folha A4 e geraram páginas de continuação.
* `ERR_ORPHAN_NOTES_DELIMITER`: Erro fatal se o delimitador `???` for utilizado sem uma lâmina aberta correspondente ou duplicado no mesmo bloco.

