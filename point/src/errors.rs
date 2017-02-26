use astrometry;
use gphoto;

error_chain! {
    links {
      Astrometry(astrometry::errors::Error, astrometry::errors::ErrorKind); 
      Gphoto(gphoto::errors::Error, gphoto::errors::ErrorKind); 
    }
    //errors {
        //GphotoCommandFailed {}
        //EmptyFile {}
    //}
}
