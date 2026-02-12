# css/CSS2/positioning/absolute-non-replaced-width-018.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-non-replaced-width-018.xht"
}
```

## style[0]

```css
<![CDATA[
  body
  {
  direction: ltr;
  margin: 8px;
  }

  div#outer-abs-pos
  {
  background-color: red;
  font: 30px/4 Ahem;
  left: auto;
  /*
  left is set to static position: it should be 8px from
  the left-hand side of document box (at body's margin-left)
  */
  position: absolute;
  right: auto;
  width: auto;
  }

  div#inner-inline-block
  {
  background-color: green;
  display: inline-block;
  max-width: 4em;
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
