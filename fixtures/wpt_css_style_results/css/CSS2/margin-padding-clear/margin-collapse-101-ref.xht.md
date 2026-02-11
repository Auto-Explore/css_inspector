# css/CSS2/margin-padding-clear/margin-collapse-101-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-101-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  table
  {
  border: solid 3px;
  border-spacing: 0;
  font-size: 50px;
  }

  td
  {
  background-color: orange;
  padding: 0;
  }

  div {margin: 1em 0;}

  .a {background-color: yellow;}

  .b {background-color: lime;}

  .c {background-color: aqua;}

  .d {background-color: blue;}
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
