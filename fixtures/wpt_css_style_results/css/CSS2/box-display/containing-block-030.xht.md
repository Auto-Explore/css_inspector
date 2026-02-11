# css/CSS2/box-display/containing-block-030.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/box-display/containing-block-030.xht"
}
```

## style[0]

```css
<![CDATA[
  div#containing-block
  {
  background-color: blue;
  height: 1in;
  padding-left: 5px;
  width: 1in;
  }

  div#sole-child-with-taller-content
  {
  background-color: orange;
  height: 2in;
  width: 0.5in;
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
