# css/CSS2/box-display/containing-block-029-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/box-display/containing-block-029-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  table
  {
  border-spacing: 0px;
  height: 96px;
  table-layout: fixed;
  width: 96px;
  }

  col#first-column {width: 72px;}

  col#second-column {width: 24px;}

  td
  {
  background-color: blue;
  padding: 0px;
  }

  tr {height: 72px;}

  tr#twenty-four {height: 24px;}

  td#orange-dot
  {
  background-color: orange;
  vertical-align: top;
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
