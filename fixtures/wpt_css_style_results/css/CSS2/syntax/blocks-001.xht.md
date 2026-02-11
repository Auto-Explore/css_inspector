# css/CSS2/syntax/blocks-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/blocks-001.xht"
}
```

## style[0]

```css

            div
            {
                \}
                "string}\" }" #div1{color:red}
                {}#div5{color: red}
                (
                    }
                    #div2{color: red}
                )
                [
                    }
                    #div3{color: red}
                ]
                [
                    (
                        ]
                    )
                    }
                    #div4{color: red}
                ]
            }
        
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Unbalanced braces.",
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
