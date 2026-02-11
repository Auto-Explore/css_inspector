# css/css-writing-modes/first-page-vlr-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/first-page-vlr-003.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-lr;
    }

  @page :right
    {
      margin-left: 0%;
    }

  @page :left
    {
      margin-left: 50%;
    }

  p
    {
      border-left: blue solid 6px;
      margin: 0;
      padding-left: 1em;
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
