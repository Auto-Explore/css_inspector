# css/CSS2/box-display/box-generation-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/box-display/box-generation-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  div#block
  {
  background-color: blue;
  width: 1in;
  }

  div#yellow-cell
  {
  background-color: yellow;
  display: table-cell;
  width: 0.5in;
  }

  div#orange-cell
  {
  background-color: orange;
  display: table-cell;
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
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
