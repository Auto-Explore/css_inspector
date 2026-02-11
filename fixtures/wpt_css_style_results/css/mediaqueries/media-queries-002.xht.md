# css/mediaqueries/media-queries-002.xht

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/media-queries-002.xht"
}
```

## style[0]

```css
<![CDATA[
   @media screen and (width) { .a { color: green; } }
   @media screen and (min-width: 1em) { .b { color: green; } }
   @media screen and (max-width: 1000000em) { .c { color: green; } }
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
