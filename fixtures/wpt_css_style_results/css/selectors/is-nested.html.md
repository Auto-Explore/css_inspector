# css/selectors/is-nested.html

```json
{
  "format_version": 3,
  "file": "css/selectors/is-nested.html"
}
```

## style[0]

```css

      /* Testing that highest specificity is chosen for class outside of :is() */
      .a+.b+.c>.e+.d {
        color: black;
        font-size: 10px;
        width: 10px;
      }
      .e:is(.b+.f, .e:is(*, .c>.e, .g, *))+.d {
        color: red;
        font-size: 20px;
      }
      .a+.b+.c>.e+.d {
        color: yellow;
      }
      /* Testing specificty of a class within :is() */
      .a+.c>.e {
        color: black;
      }
      .e:is(.b+.f, :is(.c>.e, .g)) {
        color: red;
      }
      .c>.e {
        color: black;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
