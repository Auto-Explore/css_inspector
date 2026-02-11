# css/CSS2/tables/border-conflict-w-070.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-conflict-w-070.xht"
}
```

## style[0]

```css
<![CDATA[
   table { border-collapse: collapse; }
   td {  border: 5px hidden red; height: 3em; }
   .c5 { border-width: 10px; border-style: groove; }
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
