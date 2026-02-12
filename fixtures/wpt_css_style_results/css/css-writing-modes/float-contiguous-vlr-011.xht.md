# css/css-writing-modes/float-contiguous-vlr-011.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/float-contiguous-vlr-011.xht"
}
```

## style[0]

```css
<![CDATA[
  div#reference-overlapped-red
    {
      background-color: red;
      height: 100px;
      position: absolute;
      width: 100px;
      z-index: -1;
    }

  div.floated-left
    {
      color: green;
      float: left;
      font: 20px/1 Ahem; /* computes to 20px/20px */
      writing-mode: vertical-lr;
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
