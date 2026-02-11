# css/CSS2/bidi-text/bidi-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/bidi-text/bidi-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  span
  {
  color: blue;
  line-height: 3em;
  }

  span.top-left-bottom
  {
  border-style: solid none solid solid;
  padding: 0.4em 0 0.4em 1em;
  }

  span.top-right-bottom
  {
  border-style: solid solid solid none;
  padding: 0.4em 1em 0.4em 0;
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
