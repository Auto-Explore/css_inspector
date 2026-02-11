# css/selectors/old-tests/css3-modsel-161.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-161.xml"
}
```

## style[0]

```css
<![CDATA[
  p { background: lime; }
  p   * { background: lime; }
  p > * { background: lime; }
  p + * { background: lime; }
  p ~ * { background: lime; }

  /* let's try some pseudos that are not valid CSS but are likely to
  be implemented as extensions in some UAs. These should not be
  recognised, as UAs implementing such extensions should use the
  :-vnd-ident syntax. */

  :canvas { background: red; }
  :viewport { background: red; }
  :window { background: red; }
  :menu { background: red; }
  :table { background: red; }
  :select { background: red; }
  ::canvas { background: red; }
  ::viewport { background: red; }
  ::window { background: red; }
  ::menu { background: red; }
  ::table { background: red; }
  ::select { background: red; }
]]>
```

```json
{
  "errors": 14,
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
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
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
