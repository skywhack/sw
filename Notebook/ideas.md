# Ideas

## Cloud documentation

ASDoc is automatically compiled for each package that is published into the package registry through a server container, as in the crates.io service for the Rust language.

## Scale management

Scale either the entire application or a component using the letterbox method.

## whack.gfx.\*

2D display list implemented using PIXI.js. Implement whack.gfx.\* as an optional package in the Whack registry.

- PencilStage (the main container as an UIComponent)
- DisplayObject (contains zero or more child display objects)
  - Bitmap
  - Shape
  - TextArea

## whack.gfx3d.\*

3D display implemented using THREE.js. Implement whack.gfx3d.\* as an optional package in the Whack registry.

## foam

A 2D physics engine that is a fork of an old Flash library.

## mysql

A series of packages for performing MySQL database migration and seeding and for MySQL database access using a schema.

## express

A HTTP server framework.

It embeds the CORS middleware from the NPM `cors` package.

It embeds a way of retrieving user's real IP address from an application using [@fullerstack/nax-ipware](https://github.com/neekware/fullerstack/tree/main/libs/nax-ipware).

It embeds handling of `multipart/form-data` using the NPM [`multer` package](https://www.npmjs.com/package/multer).

```
package
{
    import express.core.*;
    import express.util.cors.*;
    import express.util.ip.*;

    public class MyServer extends Application
    {
        private const ipware:Ipware = new Ipware();

        public function MyServer()
        {
            super();

            // allow cross-origin resource sharing
            this.use(cors());

            // retrieve IP
            this.use(function(req:Request, res:Response, next:Function):void
            {
                req.ipInfo = ipware.getClientIP(req);
                // { ip: '177.139.100.100', isPublic: true, isRouteTrusted: false }
                next();
            });

            // index
            this.get("/", function(req:Request, res:Response):void
            {
                res.send("hello world");
            });

            // listen in port 3000
            this.listen(3000);
        }
    }
}
```