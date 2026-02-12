# css/css-writing-modes/float-vlr-013.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/float-vlr-013.xht"
}
```

## style[0]

```css
<![CDATA[
  div#wrapper
    {
      background: red url("support/pattern-rg-gr-100x100.png") no-repeat;
      color: transparent;
      font: 50px/1 Ahem; /* computes to 50px/50px */
      height: 100px;
      width: 100px;
      writing-mode: vertical-lr;
    }

  div#floated-right
    {
      color: green;
      float: right;
    }

  div#floated-left
    {
      color: green;
      float: left;
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
