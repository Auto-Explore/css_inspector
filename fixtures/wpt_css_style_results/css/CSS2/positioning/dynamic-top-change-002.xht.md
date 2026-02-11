# css/CSS2/positioning/dynamic-top-change-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/dynamic-top-change-002.xht"
}
```

## style[0]

```css
<![CDATA[
    p
    {
    line-height: 1.25;
    margin: 1em 0;
    }

    strong {line-height: 1;}

    .testDiv { width: 100px; height: 100px; }
    #green { top: 4em; background: green; position: absolute; }
    #red { top: inherit; background: red; position: relative; }
    #parent { position: absolute; top: 2em; }
    #intermediate { display: table-row; }
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
