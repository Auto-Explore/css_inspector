# css/css-contain/content-visibility/content-visibility-layout-paint-containment-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-layout-paint-containment-001.html"
}
```

## style[0]

```css

  /* Selectors for content-visibility */
  #spacer_for_far_to_viewport {
      height: 300vh;
  }
  .content_visibility {
      /* Dynamic modification of content-visibility may change whether style
         containment is applied, which in turn may cause drastic invalidations
         (e.g. rebuilding counters). Make the test more robust by forcing
         style containment to always apply. */
      contain: style;
  }
  #visible {
      content-visibility: visible;
  }
  #hidden {
      content-visibility: hidden;
  }
  #auto_far {
      content-visibility: auto;
  }
  #auto_close {
      content-visibility: auto;
  }
  #visible_to_hidden {
      content-visibility: visible;
  }
  #hidden_to_visible {
      content-visibility: hidden;
  }
  #visible_to_auto {
      content-visibility: visible;
  }
  #auto_to_visible {
      content-visibility: auto;
  }

  /* Selectors for testing absolute/fixed positioning container blocks */
  #top_spacer {
      height: 100px;
      background: lightgray;
  }
  .absolute_pos {
      position: absolute;
      top: 42px;
  }
  .fixed_pos {
      position: fixed;
      top: 42px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
