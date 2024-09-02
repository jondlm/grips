# Grips

**This project is very immature, use at your own risk.**

A micro tool for building static websites. First create a `grips.json` in the root dir of your site. Example:

```json
{
  "source": "src",
  "target": "www",
  "extensions_to_copy": ["png"],
  "vars": {
    "foo": "bar"
  }
}
```

Then put any `.hbs.html` in your `src` directory. The templates will have
access to any key/value pairs you put in `vars`. Running `grips` will
recursively render all Handlebars templates and copy `.png` files from `src`
into `www`.

## TODO

- [ ] improve error handling and messages
- [ ] figure out partials
- [ ] support dev & prod builds, allow different vars for each, needed for `base_url`
- [ ] release binaries with gh actions

