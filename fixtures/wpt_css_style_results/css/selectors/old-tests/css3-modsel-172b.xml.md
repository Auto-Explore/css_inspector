# css/selectors/old-tests/css3-modsel-172b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-172b.xml"
}
```

## style[0]

```css
<![CDATA[
 @namespace url(http://css.example.net/);
 tests, tests * { display: block; color: green; }
 testA[|attribute] { color: red; }
 testB[|attribute="fail"] { color: red; }
 testC[|attribute~="fail"] { color: red; }
 testD[|attribute^="fail"] { color: red; }
 testE[|attribute*="fail"] { color: red; }
 testF[|attribute$="fail"] { color: red; }
 testG[|attribute|="fail"] { color: red; }
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
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    },
    {
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
