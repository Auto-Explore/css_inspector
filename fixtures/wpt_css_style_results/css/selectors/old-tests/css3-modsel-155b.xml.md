# css/selectors/old-tests/css3-modsel-155b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-155b.xml"
}
```

## style[0]

```css
<![CDATA[
  p { background: lime; }
  .two\ words { background: red; }

  /* the "." and "~=" forms match on a space separated list of words.
  In such a list, a word containing a space can never match, since it
  would by definition be two words. */

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
