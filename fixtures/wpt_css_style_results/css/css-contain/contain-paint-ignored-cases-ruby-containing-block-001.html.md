# css/css-contain/contain-paint-ignored-cases-ruby-containing-block-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-paint-ignored-cases-ruby-containing-block-001.html"
}
```

## style[0]

```css

      rb,
      rbc,
      rt,
      rtc {
        contain: paint;
        background-color: yellow;
        font-size: 2em;
      }
      rbc {
        display: ruby-base-container;
      }
      .contained {
        width: 50px;
        height: 10px;
        background-color: blue;
        top: 0;
        left: 0;
        position: fixed;
      }
      .wrapper {
        display: inline-block;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
