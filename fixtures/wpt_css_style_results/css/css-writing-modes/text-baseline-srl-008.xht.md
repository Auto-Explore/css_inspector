# css/css-writing-modes/text-baseline-srl-008.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-baseline-srl-008.xht"
}
```

## style[0]

```css
<![CDATA[
    div#srl
    {
      color: fuchsia;
      font: 60px/1.5 Ahem; /* computes to 60px/90px */
      writing-mode: sideways-rl;
    }

    span#fuchsia120
    {
      font-size: 2em; /* computes to 120px */
    }

    span#fuchsia30
    {
      font-size: 0.5em; /* computes to 30px */
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
