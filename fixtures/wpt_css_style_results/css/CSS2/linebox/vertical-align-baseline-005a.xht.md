# css/CSS2/linebox/vertical-align-baseline-005a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/linebox/vertical-align-baseline-005a.xht"
}
```

## style[0]

```css
<![CDATA[
  body {margin: 8px;}

  p
  {
  font: 1em/1.25 serif;
  margin: 1em 0em;
  }

  div#wrapper
  {
  background-color: yellow;
  /*
  The sole purpose for such yellow background
  is to help delimit and visually identify the
  content area of the block container box.
  */

  color: black;
  font: 50px/1 Ahem;
  /*
  So that 50px / 5 == 10 without remainder;
  that way, the accurate position of baseline
  does not imply fractional pixel.
  Also, the height of the "p" glyph will not
  create fractional pixel either.
  */
  }

  div#inline-block-with-overflow-hidden
  {
  color: blue;
  display: inline-block;
  margin-bottom: 99px;
  height: 2em;
  overflow: hidden;
  width: 2em;
  }

  img
  {
  position: absolute;
  top: 72px;

  /*

    16px : max(body's margin-top, p's margin-top) == max(8px, 16px)
  +
    20px : p's first line box height
  +
    20px : p's second line box height
  +
    16px : p's margin-bottom
  ========
    72px

  */
  }
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
