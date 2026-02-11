# css/css-writing-modes/float-vrl-008.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/float-vrl-008.xht"
}
```

## style[0]

```css
<![CDATA[
  div#wrapper
    {
      background: red url("support/pattern-gg-gr-100x100.png") no-repeat;
      color: green;
      font: 50px/1 Ahem; /* computes to 50px/50px */
      height: 100px;
      width: 100px;
      writing-mode: vertical-rl;
    }

  div#floated-left
    {
      color: transparent;
      float: left;
    }
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
