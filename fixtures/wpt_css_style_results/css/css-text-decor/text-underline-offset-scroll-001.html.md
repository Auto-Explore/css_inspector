# css/css-text-decor/text-underline-offset-scroll-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-underline-offset-scroll-001.html"
}
```

## style[0]

```css

        /*
         * Testing to make sure that positioning the underline
         * outside of the scrollframe does not create scrollable
         * overflow and is not visible outside of the div
         */
        #scroll{
            border: black dashed;
            overflow-y: auto;
            height: 5em;
            width: 20em;
            font: 20px/1 Ahem;
        }

        #text{
            color: transparent;
            text-decoration: red underline;
            text-decoration-skip-ink: none;
            text-underline-offset: 5em;
            text-underline-position:from-font;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
