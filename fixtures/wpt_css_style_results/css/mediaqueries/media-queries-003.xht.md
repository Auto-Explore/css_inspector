# css/mediaqueries/media-queries-003.xht

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/media-queries-003.xht"
}
```

## style[0]

```css
<![CDATA[
   @media screen and (color) { .a { color: green; } }
   @media screen and (min-color: 1) { .b { color: green; } }
   @media screen and (max-color: 4096) { .c { color: green; } }
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
