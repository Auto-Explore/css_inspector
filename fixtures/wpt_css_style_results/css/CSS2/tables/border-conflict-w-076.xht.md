# css/CSS2/tables/border-conflict-w-076.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-conflict-w-076.xht"
}
```

## style[0]

```css
<![CDATA[
   table { border-collapse: collapse; }
   td {  border: 5px outset lime; height: 3em; }
   .c5 { border-width: 10px; border-style: groove; }
   .c2 { border-bottom-color: red; }
   .c6 { border-left-color: red; }
   .c8 { border-top-color: red; }
   .c4 { border-right-color: red; }
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
