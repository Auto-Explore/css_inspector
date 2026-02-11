# css/CSS2/fonts/font-family-invalid-characters-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/fonts/font-family-invalid-characters-004.xht"
}
```

## style[0]

```css

            <![CDATA[
            #div3
            {
                background: red;
            }
            #div1
            {
                font-family: test]foo, Ahem;
            }
            #div2
            {
                font-family: test[foo, Ahem;
            }
            body
            {
                background: red;}]
            }
            #div3
            {
                background: transparent;
            }
            #div3
            {
                font-family: test[foo], Ahem;
            }
            ]]>
        
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
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
