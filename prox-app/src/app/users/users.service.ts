import { Injectable } from '@angular/core';
import { Response, Headers, RequestOptions, Http } from '@angular/http';
import { Observable } from 'rxjs/Rx';

import 'rxjs/add/operator/map';

import { User } from './user';
import { USERS } from './users-mock';

const usersUrl = 'http://localhost:8000/api/users';

@Injectable()
export class UsersService {

  constructor(private http: Http) {  }

  getMockUsers(): User[] {
    return USERS;
  }

  getRemoteUsers(): Observable<User[]> {
    return this.http.get(usersUrl)
      .map((res:Response) => res.json())
      .catch((error:any) => Observable.throw(error.json().error || 'Server error'));
  }

  addRemoteUser(user: User): Observable<User> {
    let headers = new Headers({ 'Content-Type': 'application/json' });
    let options = new RequestOptions({ headers: headers });

    return this.http.post(`${usersUrl}/new`, {name: user.name, age: user.age}, options)
      .map((res:Response) => res.json())
      .catch((error:any) => Observable.throw(error.json().error || 'Server error'));
  }

  deleteUser(id: number): Observable<User> {
    let options = new RequestOptions();
    return this.http.delete(`${usersUrl}/${id}`, options)
      .map((res:Response) => res.json())
      .catch((error:any) => Observable.throw(error.json().error || 'Server error'));
  }

}
