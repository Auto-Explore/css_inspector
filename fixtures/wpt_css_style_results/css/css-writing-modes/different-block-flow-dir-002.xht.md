# css/css-writing-modes/different-block-flow-dir-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/different-block-flow-dir-002.xht"
}
```

## style[0]

```css
<![CDATA[
  span#outer
  {
  vertical-align: top;
  writing-mode: vertical-rl;
  }

  span#inner
  {
  background-color: green;
  color: green;
  height: 6.25em;
  vertical-align: top;
  width: 6.25em;
  writing-mode: horizontal-tb;
  }

  div#reference-overlapped-red
  {
  background-color: red;
  bottom: 6.25em;
  height: 6.25em;
  position: relative;
  width: 6.25em;
  z-index: -1;
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
