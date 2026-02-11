# css/css-writing-modes/baseline-inline-replaced-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/baseline-inline-replaced-002.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      background-color: red;
      font: 96px/1 Ahem; /* computes to 96px/96px */
      height: 99px; /* the height of the cat image */
      writing-mode: vertical-rl;
    }

  img
    {
      vertical-align: baseline;
    }
  /* In vertical writing mode, the central baseline is used as the dominant baseline;
  here, the central baseline is assumed to be halfway between the under and over
  logical margin edges of the inline replaced element box. */

  /* cat.png has an intrinsic height of 99px and an intrinsic width of 98px */
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
