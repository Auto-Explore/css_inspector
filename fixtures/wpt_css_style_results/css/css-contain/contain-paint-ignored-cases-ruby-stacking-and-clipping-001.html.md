# css/css-contain/contain-paint-ignored-cases-ruby-stacking-and-clipping-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-paint-ignored-cases-ruby-stacking-and-clipping-001.html"
}
```

## style[0]

```css

      div {
        position: relative;
      }
      rb,
      rbc,
      rt,
      rtc {
        contain: paint;
      }
      rbc {
        display: ruby-base-container;
      }
      .contained {
        z-index: 5;
        width: 70px;
        height: 10px;
        background-color: blue;
        margin-left: -25px;
      }
      .background {
        background-color: yellow;
        height: 50px;
        width: 50px;
        position: fixed;
        z-index: 2;
      }
      .group {
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
