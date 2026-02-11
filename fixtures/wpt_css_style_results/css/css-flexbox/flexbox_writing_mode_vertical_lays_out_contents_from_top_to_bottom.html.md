# css/css-flexbox/flexbox_writing_mode_vertical_lays_out_contents_from_top_to_bottom.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_writing_mode_vertical_lays_out_contents_from_top_to_bottom.html"
}
```

## style[0]

```css

        .container {
            display: flex;

            flex-wrap: wrap;
            align-content: flex-start;

            writing-mode: vertical-rl;

            width: 200px;
            height: 200px;
        }
        .item {
            width: 100px;
            height: 100px;
        }

        .item.one
        {
            background: red;
        }

        .item.two
        {
            background: yellow;
        }

        .item.three
        {
            background: green;
        }

        .item.four
        {
            background: blue;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
