# css/CSS2/fonts/font-family-invalid-characters-005.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/fonts/font-family-invalid-characters-005.xht"
}
```

## style[0]

```css

            <![CDATA[
            #div1
            {
                background: red;
                font-family: test"foo, Ahem;
                color: red;
                background: transparent;
            }
            #div2
            {
                font-family: test"foo", Ahem;
            }
            #div3
            {
                background: red;
                font-family: test'foo, Ahem;
                color: red;
                background: transparent;
            }
            #div4
            {
                font-family: test'foo', Ahem;
            }
            ]]>
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Invalid input.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
