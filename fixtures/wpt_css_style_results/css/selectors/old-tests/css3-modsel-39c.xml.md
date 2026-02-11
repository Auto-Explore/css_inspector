# css/selectors/old-tests/css3-modsel-39c.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-39c.xml"
}
```

## style[0]

```css
<![CDATA[p::first-letter { color: lime; font-size: xx-large; }
 p::before { color: red; content: 'T'; }]]>
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
