# css/CSS2/visuren/anonymous-boxes-001b.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/visuren/anonymous-boxes-001b.xht"
}
```

## style[0]

```css
<![CDATA[
  body
  {
  font: 1em/1.25 serif;
  margin: 8px;
  }

  p {margin: 1em 0;}

  strong {line-height: 1;}

  div#overlapped-red
  {
  background-color: red;
  height: 100px;
  left: 8px;
  position: absolute;
  top: 122px;
  /*
    16px : max(body's margin-top, p's margin-top)
  +
    20px : p's 1st line box height
  +
    20px : p's 2nd line box height
  +
    16px : p's margin-bottom
  +
    50px : div#closest-non-anonymous-ancestor line box height
  -----------------------------------------------------------
   122px
  */
  right: 8px;
  z-index: -1;
  }

  div#closest-non-anonymous-ancestor
  {
  color: white;
  font: 2.5em/1.25 Ahem;  /* equivalent to 40px/50px in absolute units */
  height: 200px;
  }

  div#child-of-anonymous-block-box
  {
  background-color: green;
  height: 50%;
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
