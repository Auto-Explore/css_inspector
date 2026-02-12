# css/css-text-decor/text-underline-offset-zero-position.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-underline-offset-zero-position.html"
}
```

## style[0]

```css

        .green {
            color: green;
        }
        .red {
            color: red;
        }
        #test {
            font: 100px Ahem;
            color: green;
            -webkit-text-decoration-skip: none;
            text-decoration-skip-ink: none;
        }
        u {
            text-decoration: underline;
            text-decoration-color: red;
            text-decoration-thickness: 20px;
            text-underline-position: auto;
            text-underline-offset: 0px;  /* at the alphabetic baseline, per spec */
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
