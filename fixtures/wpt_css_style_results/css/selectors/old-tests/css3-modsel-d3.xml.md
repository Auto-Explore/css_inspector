# css/selectors/old-tests/css3-modsel-d3.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-d3.xml"
}
```

## style[0]

```css
<![CDATA[
   [test] { background: red; display: block; padding: 1em; }
   stub ~ [|attribute^=start]:not([|attribute~=mid])[|attribute*=dle][|attribute$=end] ~ t { background: lime; }
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
