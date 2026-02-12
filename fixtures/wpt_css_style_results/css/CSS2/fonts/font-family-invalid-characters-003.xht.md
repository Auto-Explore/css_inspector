# css/CSS2/fonts/font-family-invalid-characters-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/fonts/font-family-invalid-characters-003.xht"
}
```

## style[0]

```css

            <![CDATA[
            #div3
            {
                background: red;
                color: red;
            }
            #div1
            {
                font-family: test{foo}, Ahem;
            }
            #div2
            {
                color: red;
                font-family: test{foo, Ahem;
                background: red;
            }
            body
            {
                background: red;};
                color: inherit;
            }
            #div3
            {
                background: transparent;
            }
            #div3
            {
                font-family: test}foo, Ahem;
            }
            body
            {
               background: red;
            }
            #div3
            {
               color: inherit;
            }
            ]]>
        
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Missing ';' between declarations.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-family”.",
      "severity": "Error"
    },
    {
      "message": "Missing ';' between declarations.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
