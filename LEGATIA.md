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
| `apps/photocraft-web/src/web.rs`, `crates/ui-egui/src/state.rs` | Só o tema: a versão web abria sempre no Pro (`ThemeKind::Pro` fixo, que também grava a preferência) e o estado inicial era o ProMedium; ambos passam a ser o Studio. |
| `crates/ui-egui/src/panels/title_bar.rs`, `canvas/start_screen.rs` | Só ganchos: o nome "LegatIA Photo" na barra e na tela inicial (`legatia::NAME`), "based on PhotoCraft" no lugar de "open source", e o rodapé `legatia::start_footer` no lugar do botão do Discord e dos links do PhotoCraft/GitHub/ArtCraft (que levam à comunidade do projeto original). |
| `crates/ui-egui/src/lib.rs` | `pub mod legatia;` |
| `assets/app-icon/hicolor/128x128/apps/ai.storyteller.photocraft.png`, `assets/app-icon/legatia.svg`, `legatia-512.png` | O ícone da LegatIA (o "/IA" branco num quadrado arredondado preto): o PNG de 128 px é o que a barra de título embute. |
| `apps/photocraft-web/index.html`, `apps/photocraft-web-boot/index.html` | Título "LegatIA Photo", fundo preto desde o primeiro quadro, "Carregando o editor…". |
| `docs/brand/` | Removido: a licença da marca ArtCraft pede que forks tirem os logos. O `README.md` e o `ATTRIBUTION.md` dizem isso. |
| `.github/workflows/legatia-web.yml`, `LEGATIA_BASE`, este arquivo | A atualização automática e o build. |

O que **ainda não** foi trazido do LegatIA Design (próximas etapas): o vidro fosco nos painéis, a
ponte com a página em volta (`host`: salvar na LegatIA, arquivos do projeto) e a Satoshi na interface.
O PhotoCraft desenha o documento com um motor próprio em blocos, então o vidro precisa de um estudo à parte.

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
