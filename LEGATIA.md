# LegatIA Photo (fork do PhotoCraft)

Este fork é o [PhotoCraft](https://github.com/storytold/photocraft) com a cara da LegatIA, para o editor
de imagens do LegatIAdm. É o irmão do [LegatIA Design](https://github.com/Lulindos/figcraft) (fork do
FigCraft) e usa o mesmo método. O branch **`legatia`** é a release do PhotoCraft indicada em
`LEGATIA_BASE` mais poucos commits nossos. O `main` deste fork fica igual ao original.

## O que muda

| Onde | O quê |
|---|---|
| `crates/ui-egui/src/legatia.rs` | As cores da LegatIA (preto, bordas finas, azul #006FFE, cartões de 18 px) sobre os tokens do PhotoCraft. Só vale para os temas **Studio** (escuro e claro); Pro, Classic, Adwaita e os demais ficam como estão. Arquivo só nosso, com testes. |
| `crates/ui-egui/src/theme.rs` | Só ganchos: `Tokens::for_kind` passa o resultado por `legatia::tokens` (a função de antes virou `for_kind_base`); o tema padrão é o Studio; os rótulos dizem "LegatIA (Dark)" e "LegatIA (Light)". |
| `crates/engine/src/prefs.rs` (e o teste dos padrões em `prefs/tests.rs`) | Só o padrão: o tema inicial das preferências é o Studio (LegatIA escuro). Sem isso, quem abre o editor pela primeira vez vê o Pro (cinza). |
| `crates/ui-egui/src/panels/dock.rs` | Só um gancho: `legatia::dock_frame` deixa 6 px em volta do dock e do canvas (a janela, a barra de cima), para os cartões flutuarem como no LegatIA Design. |
| `crates/ui-egui/src/panels/panel_docking.rs` | Dois ganchos: `legatia::dock_style` (os cartões do dock ficam separados por 6 px, como no LegatIA Design) e `legatia::canvas_or_estimate` (o cartão Properties do tema Studio era posto contra um canvas de 800x600 que só vale até abrir um documento, e ficava no meio da tela inicial; agora usa uma estimativa pelo tamanho da janela). |
| `apps/photocraft-web/src/web.rs`, `crates/ui-egui/src/state.rs` | Só o tema: a versão web abria sempre no Pro (`ThemeKind::Pro` fixo, que também grava a preferência) e o estado inicial era o ProMedium; ambos passam a ser o Studio. |
| `crates/ui-egui/src/panels/title_bar.rs`, `canvas/start_screen.rs` | Só ganchos (`title_bar` também anota o tamanho da janela em `legatia::note_screen`): o nome "LegatIA Photo" na barra e na tela inicial (`legatia::NAME`), "based on PhotoCraft" no lugar de "open source", e o rodapé `legatia::start_footer` no lugar do botão do Discord e dos links do PhotoCraft/GitHub/ArtCraft (que levam à comunidade do projeto original). |
| `Cargo.toml`, `Cargo.lock` | Um `[patch."https://github.com/storytold/craft-libs"]` leva os pacotes `craft-*` que o PhotoCraft usa para o fork [Lulindos/craft-libs](https://github.com/Lulindos/craft-libs), que dá ao dock do `craft-ui` o `DockStyle::corner_radius` (cartões de 18 px). O `legatia::dock_style` liga o raio. Ao subir o `rev` do craft-libs do PhotoCraft, refaça o fork em cima dele (ver o LEGATIA.md de lá). |
| `crates/ui-egui/src/lib.rs` | `pub mod legatia;` |
| `assets/app-icon/hicolor/128x128/apps/ai.storyteller.photocraft.png`, `assets/app-icon/legatia.svg`, `legatia-512.png` | O ícone da LegatIA (o "/IA" branco num quadrado arredondado preto): o PNG de 128 px é o que a barra de título embute. |
| `apps/photocraft-web/index.html`, `apps/photocraft-web-boot/index.html` | Título "LegatIA Photo", fundo preto desde o primeiro quadro, "Carregando o editor…". |
| `docs/brand/` | Removido: a licença da marca ArtCraft pede que forks tirem os logos. O `README.md` e o `ATTRIBUTION.md` dizem isso. |
| `.github/workflows/legatia-web.yml`, `LEGATIA_BASE`, este arquivo | A atualização automática e o build. |

O que **ainda não** foi trazido do LegatIA Design (próximas etapas; os cantos de 18 px dos painéis já vieram):

- **O vidro fosco.** Depende de o desenho do documento passar por baixo dos painéis: o PhotoCraft
  desenha o documento num motor próprio em blocos, então é um estudo à parte.
- A ponte com a página em volta (`host`: salvar na LegatIA, arquivos do projeto) e a Satoshi na interface.

## Atualização automática

`.github/workflows/legatia-web.yml` roda todo dia às 06:37 (e quando chamado em Actions ▸ LegatIA web):

1. Se o PhotoCraft tem release mais nova que `LEGATIA_BASE`, reaplica os commits da LegatIA em cima dela
   (`git rebase --onto <nova> <base> legatia`) e atualiza `LEGATIA_BASE`.
2. Gera a versão web como o CI do próprio PhotoCraft (`trunk`, `packaging/web/package.sh`) e publica a
   release `<base>-legatia.<n>` com o zip e o SHA-256. As notas da release trazem o trecho pronto para o
   `photocraft.web.json` do LegatIAdm.
3. Se uma mudança deles bater nas mesmas linhas que as nossas, nada é publicado: abre uma issue
   "LegatIA: conflito ao atualizar…" com o passo a passo, e a LegatIA continua na versão anterior.

Um push no branch `legatia` (uma mudança nossa) também gera uma release nova na mesma base.

## Regras para mexer aqui

- Personalização nova vai em `legatia.rs` sempre que der; no código do PhotoCraft, só a linha que chama ela.
  Quanto menos linhas do original tocadas, menos conflito nas atualizações.
- Um token novo do PhotoCraft que a LegatIA ainda não mapeou fica com o valor deles (`..t` em `legatia.rs`),
  em vez de quebrar o build: vale olhar `Tokens` em `theme.rs` a cada atualização.
- Não suba arquivos de fonte com licença que proíba redistribuir: o repositório e as releases são públicos.
- Não use o nome nem os logos da ArtCraft no nome, no ícone ou na divulgação do que sai daqui (ver
  `ATTRIBUTION.md`); pode dizer, em texto, que é baseado no PhotoCraft da equipe ArtCraft.
