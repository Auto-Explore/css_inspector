# css/css-fonts/lang-attribute-affects-rendering-of-second-text-run-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/lang-attribute-affects-rendering-of-second-text-run-ref.html"
}
```

## style[0]

```css

    * { font-size: 50px }

    @font-face {
      font-family: test-font-family;
      /* <Lato-Medium.ttf> provides different ligatures for English and
         Turkish. */
      src: url(/fonts/Lato-Medium.ttf);
    }

    div { font-family: test-font-family; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
