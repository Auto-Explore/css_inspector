# css/selectors/old-tests/css3-modsel-160.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-160.xml"
}
```

## style[0]

```css
<![CDATA[
  p { background: lime; }
  p:subject { background: red; } /* this is not valid CSS, and if UAs
  implemented the experimental :subject pseudo-class they should have
  used the :-vnd-ident syntax. */
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
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
