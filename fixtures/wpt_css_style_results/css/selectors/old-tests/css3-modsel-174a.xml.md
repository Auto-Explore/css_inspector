# css/selectors/old-tests/css3-modsel-174a.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-174a.xml"
}
```

## style[0]

```css
<![CDATA[
 tests, tests * { display: block; color: red; }
 testA[*|attribute="pass"] { color: green; }
 testB[*|attribute="pass"] { color: green; }
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
