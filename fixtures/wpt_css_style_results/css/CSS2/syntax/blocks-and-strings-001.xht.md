# css/CSS2/syntax/blocks-and-strings-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/blocks-and-strings-001.xht"
}
```

## style[0]

```css

            div
            {
                "this is a string]}""[{\"'";  /*should be parsed as a string but be ignored*/
                {{}}[]'';                     /*should be parsed as nested blocks and a string but be ignored*/
                color: green;
            }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
