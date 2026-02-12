# css/css-writing-modes/central-baseline-alignment-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/central-baseline-alignment-003.xht"
}
```

## style[0]

```css
<![CDATA[
  div#textorient-mixed
    {
      color: orange;
      font: 60px/1.5 Ahem; /* computes to 60px/90px */
      height: 4em;
      text-orientation: upright;
      writing-mode: vertical-lr;
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
