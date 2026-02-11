# css/CSS2/selectors/default-attribute-selector-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/selectors/default-attribute-selector-004.xht"
}
```

## style[0]

```css
<![CDATA[
  table {border-spacing: 0px;}

  td
  {
  height: 100px;
  padding: 0px;
  width: 100px;
  }

  colgroup {background-color: green;}

  colgroup[span] {background-color: red;}

  colgroup[span="1"] {background-color: green;}
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
