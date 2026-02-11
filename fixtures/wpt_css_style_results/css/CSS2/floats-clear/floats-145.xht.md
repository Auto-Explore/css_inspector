# css/CSS2/floats-clear/floats-145.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/floats-145.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      background-color: blue;
      float: left;
      font: 30px/3 Ahem; /* computes to 30px/90px */
    }

  span
    {
      background-color: white;
      color: white;
      float: left;
      margin-bottom: 1em;
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
