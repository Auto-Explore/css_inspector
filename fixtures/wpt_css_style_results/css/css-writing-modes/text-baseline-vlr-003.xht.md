# css/css-writing-modes/text-baseline-vlr-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-baseline-vlr-003.xht"
}
```

## style[0]

```css
<![CDATA[
    div#lr-mixed
    {
      color: orange;
      font: 60px/1.5 Ahem; /* computes to 60px/90px */
      writing-mode: vertical-lr;
      text-orientation: mixed;
    }

    span#blue120
    {
      color: blue;
      font-size: 2em; /* computes to 120px */
    }

    span#orange30
    {
      font-size: 0.5em; /* computes to 30px */
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
