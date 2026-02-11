# css/css-writing-modes/vertical-alignment-srl-030.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/vertical-alignment-srl-030.xht"
}
```

## style[0]

```css
<![CDATA[
    div#srl
    {
      color: orange;
      font: 60px/3 Ahem; /* computes to 60px/180px */
      writing-mode: sideways-rl;
    }

    span#orange30
    {
      font-size: 0.5em; /* computes to 30px */
      vertical-align: text-top;
    }

    ]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
