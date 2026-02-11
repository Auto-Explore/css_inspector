# css/CSS2/tables/separated-border-model-003a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/separated-border-model-003a.xht"
}
```

## style[0]

```css
<![CDATA[
  div#overlapped-red
  {
  background-color: red;
  height: 100px;
  left: auto;
  position: absolute;
  top: auto;
  width: 100px;
  z-index: -1;
  }

  table
  {
  background-color: red;
  border-collapse: separate;
  border-spacing: 0px;
  table-layout: fixed;
  width: 100px;
  }

  td
  {
  background-color: green;
  color: green;
  font: 20px/1 Ahem;
  padding: 15px 1px;
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
