=========================
``cat`` but with attitude
=========================

This project is an experiental project I started to help me learn rust. Use it at your own risk. I'll be updating it, adding features, tests, etc as I find the time to do so.

To build this project you'll need the lastest version of `rust <https://www.rust-lang.org/en-US/downloads.html>`_ and `cargo <https://github.com/rust-lang/cargo/>`_.

Usage
=====

If you want to replace the existing ``cat`` program that comes with almost all linux distrubions and Mac OS. I recommend renaming the binary created by cargo to ``cat_bwa``.

Place the ``cat_bwa`` binary in ``/usr/bin/`` or whatever is the default for your OS. If you not sure where you place the binary run ``$ which cat`` to see where the original ``cat`` program binary is and place it in the same directory.

I recommend creating an alias for the original ``cat`` and the ``cat_bwa`` program. Could place something like this in your ``.bashrc`` or ``.zshrc``.::

    alias ocat=/usr/bin/cat
    alias cat=/usr/bin/cat_bwa

Once that is done you can use the cat command like so::

    cat some_file

You'll see the contents of the file streamed to ``stdout`` (console window).

At some point the cat will meow at you instead of streaming the given file contents to ``stdout``. At that point you need to give ``cat`` a treat::

    cat some_file +treat

Cat's State
===========

The ``cat`` program's state is stored in ``/tmp/cat.txt``. In the file is a single number from 0 to 100. 
