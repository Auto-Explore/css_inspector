# css/CSS1/list-marker-inside-whitespace-001.xml

```json
{
  "format_version": 3,
  "file": "css/CSS1/list-marker-inside-whitespace-001.xml"
}
```

## style[0]

```css
<![CDATA[
   ul {
     margin: 1em 5em; border: solid black; padding: 0; width: 6em;
     font: 15px/1 Ahem; color: lime; background: red url(support/css1test566a.png) no-repeat;
     list-style: upper-alpha inside;
   }
   li { margin: 0; padding: 0; }
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
