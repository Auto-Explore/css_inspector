# css/CSS2/visuren/anonymous-boxes-001a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/visuren/anonymous-boxes-001a.xht"
}
```

## style[0]

```css
<![CDATA[
  body {margin: 8px;}

  div#overlapped-red
  {
  background-color: red;
  height: 100px;
  left: 208px;
  /*
      8px : body's margin-left
  +
    160px : 4 characters of "Some" 40px wide each
  +
     40px : one blank white space of 40px wide
  --------------------------------------------------
    208px
  */
  position: absolute;
  width: 100px;
  z-index: -1;
  }

  div#closest-non-anonymous-ancestor
  {
  color: white;
  font: 2.5em/1.25 Ahem;  /* equivalent to 40px/50px in absolute units */
  height: 200px;
  }

  img#child-of-anonymous-block-box {height: 50%;}
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
